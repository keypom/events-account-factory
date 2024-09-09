export const SIGNER_ACCOUNT = "benjiman.testnet";
export const NEW_FACTORY = true;
export const EXISTING_FACTORY = `1725893914604-factory.testnet`;
export const ADMIN_ACCOUNTS = [SIGNER_ACCOUNT];
export const SHOULD_CREATE_SPONSORS = true;
export const TICKET_URL_BASE = "http://localhost:5173/tickets/ticket/ga_pass#";
export const TICKET_DATA = {
  ga_pass: {
    startingNearBalance: "0.01",
    startingTokenBalance: "50",
    accountType: "Basic",
  },
};
export const SPONSOR_DATA = [
  {
    accountName: "proximity",
    startingNearBalance: "0.01",
    startingTokenBalance: "50",
    accountType: "Sponsor",
  },
];
