module.exports = {
  root: true,
  parser: '@typescript-eslint/parser',
  extends: [
    'standard',
    'eslint:recommended',
    'plugin:@typescript-eslint/eslint-recommended',
    'plugin:import/errors',
    'plugin:import/warnings',
    'plugin:promise/recommended',
    'plugin:react/recommended',
    'plugin:react/jsx-runtime',
    'plugin:jsx-a11y/recommended',
    'plugin:tailwindcss/recommended',
    'prettier',
  ],
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
  },
  plugins: [
    'prettier',
    '@typescript-eslint',
    'react',
    'jsx-a11y',
    'simple-import-sort',
    'listeners',
  ],
  rules: {
    'listeners/no-missing-remove-event-listener': 'error',
    'listeners/matching-remove-event-listener': 'error',
    'listeners/no-inline-function-event-listener': 'error',
    '@typescript-eslint/no-unused-vars': ['error'],
    'simple-import-sort/imports': [
      'error',
      {
        groups: [['^react$', '^@?\\w']],
      },
    ],
    'simple-import-sort/exports': 'error',
    'prettier/prettier': [
      'error',
      {
        endOfLine: 'auto',
        trailingComma: 'all',
        singleQuote: true,
        semi: true,
      },
    ],
    'react/self-closing-comp': ['error'],
  },
  settings: {
    'import/resolver': {
      node: {
        extensions: ['.js', '.jsx', '.ts', '.tsx'],
      },
    },
    react: {
      version: 'detect',
    },
  },
};
