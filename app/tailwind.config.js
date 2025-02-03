const colors = require("tailwindcss/colors")

export default {
	content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
	theme: {
		extend: {
			colors: {
				brand: colors.purple,
				neutral: colors.zinc,
				error: colors.red,
				success: colors.green,
				warning: colors.amber,
			  },
		},
	},
	plugins: [],
};
