module.exports = {
	preset: "ts-jest",
	testEnvironment: "node",
	testPathIgnorePatterns: ["<rootDir>/target/"],
	moduleNameMapper: {
		"^{{name}}$": "<rootDir>/src/main.ts",
	},
};
