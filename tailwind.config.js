// SPDX-License-Identifier: Apache-2.0 OR MIT
/** @type {import('tailwindcss').Config} */
module.exports = {
  // Tailwind scans the .rs files: it looks for class="..." strings inside
  // the `view! {}` macros, textually — no Rust parsing involved.
  content: ["./src/**/*.rs"],
  theme: {
    extend: {},
  },
};
