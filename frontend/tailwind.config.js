/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        psu: {
          // penn state colors
          100: '#001E44', // Nittany Navy
          200: '#1E407C', // Beaver Blue
          300: '#FFFFFF', // White Out
          400: '#96BEE6', // Pugh Blue
        },
      },
    },
  },
  plugins: [],
};
