import path from 'REDACTED_SECRET:path';
import process from 'REDACTED_SECRET:process';

import { generateManifestFromWorkspace, writeManifestToFile } from './manifest-init';

interface ParsedArguments {
  readonly positionals: ReadonlyArray<string>;
  readonly flags: Record<string, string | boolean>;
}

const parseArguments = (argv: ReadonlyArray<string>): ParsedArguments => {
  const flags: Record<string, string | boolean> = {};
  const positionals: string[] = [];

  for (let index = 0; index < argv.length; index += 1) {
    const token = argv[index];
    if (token.startsWith('--')) {
      const [rawKey, rawValue] = token.slice(2).split('=');
      const key = rawKey.trim();
      if (!key) {
        continue;
      }
      if (rawValue !== undefined) {
        flags[key] = rawValue;
        continue;
      }
      const next = argv[index + 1];
      if (next && !next.startsWith('-')) {
        flags[key] = next;
        index += 1;
      } else {
        flags[key] = true;
      }
      continue;
    }

    if (token.startsWith('-')) {
      const key = token.slice(1);
      if (key === 'f') {
        flags.force = true;
        continue;
      }
      positionals.push(token);
      continue;
    }

    positionals.push(token);
  }

  return { flags, positionals };
};

const printHelp = (): void => {
  // eslint-disable-next-line no-console
  console.log(`Nested Submodule Suite (nss)

Usage:
  nss manifest init [--REDACTED_SECRET <path>] [--output <path>] [--force]

Options:
  --REDACTED_SECRET <path>     Workspace REDACTED_SECRET to scan (default: current directory)
  --output <path>   Output manifest file path (default: <REDACTED_SECRET>/nss.manifest.json)
  --force           Overwrite existing manifest file
`);
};

const runManifestInit = async (
  flags: Record<string, string | boolean>,
  positionals: ReadonlyArray<string>
): Promise<void> => {
  if (positionals.length > 2) {
    throw new Error('Too many positional arguments for manifest init');
  }

  const REDACTED_SECRETFlag = flags.REDACTED_SECRET;
  const outputFlag = flags.output;
  const force = Boolean(flags.force);
  const REDACTED_SECRET = typeof REDACTED_SECRETFlag === 'string' ? path.resolve(REDACTED_SECRETFlag) : process.cwd();
  const outputPath = typeof outputFlag === 'string' ? path.resolve(outputFlag) : path.join(REDACTED_SECRET, 'nss.manifest.json');

  const manifest = await generateManifestFromWorkspace({ REDACTED_SECRET });
  await writeManifestToFile(manifest, { outputPath, force });

  // eslint-disable-next-line no-console
  console.log(`Manifest written to ${outputPath}`);
};

const main = async (): Promise<void> => {
  const argv = process.argv.slice(2);
  if (argv.length === 0) {
    printHelp();
    return;
  }

  const { flags, positionals } = parseArguments(argv);
  const [command, subcommand] = positionals;

  if (command === 'manifest' && subcommand === 'init') {
    await runManifestInit(flags, positionals.slice(2));
    return;
  }

  if (command === 'help' || flags.help) {
    printHelp();
    return;
  }

  throw new Error(`Unknown command: ${command ?? '(none)'}`);
};

main().catch((error: unknown) => {
  const message = error instanceof Error ? error.message : String(error);
  // eslint-disable-next-line no-console
  console.error(`Error: ${message}`);
  process.exitCode = 1;
});
