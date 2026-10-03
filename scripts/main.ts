import { Command } from "@cliffy/command";
import { parse as parseToml } from "jsr:@std/toml@1";
import { join } from "jsr:@std/path@1";

interface TargetEntry {
  name: string;
  alias?: string | string[];
  aliases?: string[];
}

interface OsConfig {
  root?: boolean;
  default: string;
  name: string;
  target: TargetEntry[];
}

interface BuildOptions {
  os?: string;
  osVersion?: string;
  gameMajor: string;
  gamePatch: string;
  unstable: boolean;
  variant: "root" | "rootless";
  primary: boolean;
  registry: string;
  downloaderImage: string;
  configDir: string;
  push: boolean;
  dryRun: boolean;
}

function resolveAliases(target: TargetEntry): string[] {
  const aliases: string[] = [];
  if (target.aliases && Array.isArray(target.aliases)) {
    aliases.push(...target.aliases);
  }
  if (target.alias) {
    if (Array.isArray(target.alias)) {
      aliases.push(...target.alias);
    } else {
      aliases.push(target.alias);
    }
  }
  return Array.from(new Set(aliases));
}

function generateTags(
  osConfig: OsConfig,
  target: TargetEntry,
  opts: BuildOptions,
): string[] {
  const branchSuffix = opts.unstable ? "-unstable" : "";
  const variantSuffix = opts.variant === "rootless" ? "-rootless" : "";
  const primaryBranchTag = opts.unstable ? "unstable" : "latest";

  const isDefaultOsVersion = osConfig.default === target.name;
  const isRootOs = osConfig.root === true;

  const versionKeys = [target.name, ...resolveAliases(target)];
  const tags: string[] = [];

  // 1. Fully Qualified OS Tags (e.g. 42.19-debian-13 or 42.19-debian-trixie)
  for (const vKey of versionKeys) {
    tags.push(
      `${opts.registry}:${opts.gameMajor}.${opts.gamePatch}${branchSuffix}-${osConfig.name}-${vKey}${variantSuffix}`,
      `${opts.registry}:${opts.gameMajor}${branchSuffix}-${osConfig.name}-${vKey}${variantSuffix}`,
    );
  }

  // 2. Distro Family Alias Tags (e.g. 42.19-debian)
  if (isDefaultOsVersion) {
    tags.push(
      `${opts.registry}:${opts.gameMajor}.${opts.gamePatch}${branchSuffix}-${osConfig.name}${variantSuffix}`,
      `${opts.registry}:${opts.gameMajor}${branchSuffix}-${osConfig.name}${variantSuffix}`,
    );
  }

  // 3. Clean Un-Suffixed Default OS Tags (e.g. 42.19 or 42)
  if (isRootOs && isDefaultOsVersion) {
    tags.push(
      `${opts.registry}:${opts.gameMajor}.${opts.gamePatch}${branchSuffix}${variantSuffix}`,
      `${opts.registry}:${opts.gameMajor}${branchSuffix}${variantSuffix}`,
    );
  }

  // 4. Global Top-Level Branch Tags (e.g. latest or unstable)
  if (isRootOs && isDefaultOsVersion && opts.primary) {
    tags.push(`${opts.registry}:${primaryBranchTag}${variantSuffix}`);
  }

  return Array.from(new Set(tags));
}

async function runCommand(cmd: string[], dryRun: boolean): Promise<void> {
  console.log(`\n$ ${cmd.join(" ")}`);
  if (dryRun) return;

  const command = new Deno.Command(cmd[0], {
    args: cmd.slice(1),
    stdout: "inherit",
    stderr: "inherit",
  });

  const { success, code } = await command.output();
  if (!success) {
    console.error(`Command failed with exit code ${code}`);
    Deno.exit(code);
  }
}

async function loadOsConfig(configDir: string, osName: string): Promise<OsConfig> {
  const filePath = join(configDir, `${osName}.toml`);
  try {
    const content = await Deno.readTextFile(filePath);
    return parseToml(content) as unknown as OsConfig;
  } catch (err) {
    console.error(`Failed to load config at ${filePath}`);
    throw err;
  }
}

async function discoverOsConfigs(configDir: string): Promise<string[]> {
  const osNames: string[] = [];
  for await (const entry of Deno.readDir(configDir)) {
    if (entry.isFile && entry.name.endsWith(".toml")) {
      osNames.push(entry.name.replace(/\.toml$/, ""));
    }
  }
  return osNames;
}

await new Command()
  .name("zomboid-builder")
  .version("1.0.0")
  .description("Builds multi-OS Project Zomboid dedicated server Docker images")
  .option("-o, --os <os:string>", "Target OS family (e.g. debian)")
  .option("-v, --os-version <version:string>", "Target OS version (e.g. 13)")
  .option("-m, --game-major <major:string>", "Game major version", { default: "42" })
  .option("-p, --game-patch <patch:string>", "Game patch version", { default: "21" })
  .option("-u, --unstable [unstable:boolean]", "Build unstable branch", { default: false })
  .option("--variant <variant:string>", "Build target variant (root or rootless)", {
    default: "root",
  })
  .option("--primary [primary:boolean]", "Tag as primary top-level release (latest/unstable)", {
    default: true,
  })
  .option("-r, --registry <registry:string>", "Container registry image name", {
    default: "cliftontoasterreid/project-zomboid-dedicated-server",
  })
  .option(
    "--downloader-image <image:string>",
    "Prebuilt downloader image providing /opt/zomboid (skips the Steam download)",
    { default: "" },
  )
  .option("-c, --config-dir <dir:string>", "Path to TOML configuration directory", {
    default: "config/os",
  })
  .option("--push [push:boolean]", "Push image tags directly via buildx", { default: false })
  .option("-d, --dry-run [dryRun:boolean]", "Print tags and commands without building", {
    default: false,
  })
  .action(async (options) => {
    const opts = options as unknown as BuildOptions;

    if (opts.variant !== "root" && opts.variant !== "rootless") {
      console.error(`Invalid variant '${opts.variant}'. Must be 'root' or 'rootless'.`);
      Deno.exit(1);
    }

    const osList = opts.os ? [opts.os] : await discoverOsConfigs(opts.configDir);

    if (osList.length === 0) {
      console.error(`No OS configuration TOML files found in '${opts.configDir}'.`);
      Deno.exit(1);
    }

    for (const osName of osList) {
      const osConfig = await loadOsConfig(opts.configDir, osName);

      const targetsToBuild = opts.osVersion
        ? osConfig.target.filter((t) => t.name === opts.osVersion)
        : osConfig.target;

      if (targetsToBuild.length === 0) {
        console.error(`OS version '${opts.osVersion}' not found in ${osName}.toml`);
        Deno.exit(1);
      }

      for (const target of targetsToBuild) {
        const tags = generateTags(osConfig, target, opts);

        console.log(`\n==================================================`);
        console.log(` Target OS      : ${osConfig.name}:${target.name}`);
        console.log(` Variant        : ${opts.variant}`);
        console.log(` Game Version   : ${opts.gameMajor}.${opts.gamePatch}${opts.unstable ? " (unstable)" : ""}`);
        console.log(` Generated Tags :`);
        tags.forEach((t) => console.log(`   - ${t}`));
        console.log(`==================================================`);

        const baseDockerfile = `docker/${osConfig.name}/${target.name}/Dockerfile`;
        const baseImageTag = `zomboid-base:${osConfig.name}-${target.name}`;

        // Step 1: Build OS Base Image
        const baseBuildCmd = [
          "docker",
          "build",
          "-t",
          baseImageTag,
          "-f",
          baseDockerfile,
          ".",
        ];
        await runCommand(baseBuildCmd, opts.dryRun);

        // Step 2: Build App Image with All Tags & Multi-stage Target
        const appBuildCmd = [
          "docker",
          "buildx",
          "build",
          "--target",
          opts.variant,
          "--build-arg",
          `BASE_IMAGE=${baseImageTag}`,
          "--build-arg",
          `GAME_MAJOR=${opts.gameMajor}`,
          "--build-arg",
          `GAME_PATCH=${opts.gamePatch}`,
          "--build-arg",
          `IS_UNSTABLE=${opts.unstable}`,
          "-f",
          "docker/app/Dockerfile",
        ];

        if (opts.downloaderImage) {
          appBuildCmd.push("--build-arg", `DOWNLOADER_IMAGE=${opts.downloaderImage}`);
        }

        for (const tag of tags) {
          appBuildCmd.push("-t", tag);
        }

        if (opts.push) {
          appBuildCmd.push("--push");
        }

        appBuildCmd.push(".");
        await runCommand(appBuildCmd, opts.dryRun);
      }
    }
  })
  .parse(Deno.args);