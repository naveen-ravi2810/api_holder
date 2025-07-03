/** @type {import('tailwindcss').Config} */

import defaultColors from "./utils/colors";

// import { ColorMap } from './utils/mapper';
module.exports = {
  content: [
    "./components/**/*.{vue,js}",
    "./layouts/**/*.vue",
    "./pages/**/*.vue",
    "./app.vue",
    "./plugins/**/*.{js,ts}",
    "./nuxt.config.{js,ts}",
  ],
  theme: {
    extend: {
      colors: {
        ...defaultColors,
      },
    },
  },
  plugins: [],
};
