module.exports = {
    content: ['./src/**/*.{vue,js,ts}'],
    plugins: [require('daisyui')],
    daisyui: {
        themes: [{
            syncboard: {
            ...require("daisyui/src/theming/themes")["dark"],
              "primary": "#ffa300",
              "base-100": "#1c1b1f",
              "base-200": "#2e2d30",
              "base-300": "#403f41",
            },
        }],
    },
};  