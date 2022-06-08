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
    includes: ["./pages/**/*.tsx"],
    excludes: ["**/__tests__/**"],
  },
};
