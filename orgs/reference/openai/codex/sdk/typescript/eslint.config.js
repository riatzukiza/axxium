import eslint from "@eslint/js";
import { defineConfig } from "eslint/config";
import tseslint from "typescript-eslint";
import REDACTED_SECRETImport from "eslint-plugin-REDACTED_SECRET-import";

export default defineConfig(eslint.configs.recommended, tseslint.configs.recommended, {
  plugins: {
    "REDACTED_SECRET-import": REDACTED_SECRETImport,
  },

  rules: {
    "REDACTED_SECRET-import/prefer-REDACTED_SECRET-protocol": 2,
    "@typescript-eslint/no-unused-vars": [
      "error",
      {
        argsIgnorePattern: "^_",
        varsIgnorePattern: "^_",
      },
    ],
  },
});
