module.exports = {
  client: {
    service: {
      name: "bgg",
      url: "http://localhost:4000/graphql",
      // optional headers
      headers: {},
      // optional disable SSL validation check
      skipSSLValidation: true,
    },
    includes: ["./pages/**/*.{js,ts,jsx,tsx,graphql}", "./components/**/*.{js,ts,jsx,tsx,graphql}"],
    excludes: ["**/__tests__/**"],
  },
};
