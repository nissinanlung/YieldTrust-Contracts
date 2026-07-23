const globals = require(\"globals\");

/** @type {import(\"eslint\").Linter.Config[]} */
module.exports = [
  {
    ignores: [\"node_modules/**\", \"target/**\", \"dist/**\"],
  },
  {
    files: [\"**/*.js\", \"**/*.mjs\"],
    languageOptions: {
      ecmaVersion: 2022,
      sourceType: \"commonjs\",
      globals: {
        ...globals.node,
      },
    },
    rules: {
      \"no-undef\": \"error\",
      \"no-unused-vars\": [\"warn\", { argsIgnorePattern: \"^_|next\" }],
      \"no-console\": \"off\",
      \"prefer-const\": \"error\",
      \"no-var\": \"error\",
      \"eqeqeq\": [\"error\", \"always\"],
      \"no-constant-condition\": \"error\",
      \"no-debugger\": \"error\",
      \"no-duplicate-case\": \"error\",
      \"no-empty\": \"error\",
      \"no-redeclare\": \"error\",
      \"no-shadow\": \"warn\",
      \"no-unreachable\": \"error\",
    },
  },
];
