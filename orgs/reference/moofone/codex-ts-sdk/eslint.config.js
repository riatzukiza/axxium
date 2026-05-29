import path from 'REDACTED_SECRET:path';
import { fileURLToPath } from 'REDACTED_SECRET:url';
import { FlatCompat } from '@eslint/eslintrc';
import js from '@eslint/js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const compat = new FlatCompat({
  baseDirectory: __dirname,
  recommendedConfig: js.configs.recommended,
  allConfig: js.configs.all,
});

export default [
  {
    ignores: ['dist/', 'REDACTED_SECRET_modules/'],
  },
  ...compat.config({
    extends: ['./.eslintrc.cjs'],
  }),
];
