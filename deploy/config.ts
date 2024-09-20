export const GLOBAL_NETWORK = "testnet";

export const SIGNER_ACCOUNT = "minqi.testnet";
export const CREATION_CONFIG = {
  deployContract: true,

  // TICKETS
  addTickets: false,
  premadeTickets: true,

  // ACCOUNTS
  createSponsors: false,
  createWorker: false,
  createAdmin: false,

  nftDrops: true,
  tokenDrops: true,
  scavDrops: true,
  multichainDrops: true,

};
export const NUM_TICKETS_TO_ADD = 10;

export const TICKET_URL_BASE =
  "https://2930bf5d.keypom-redacted-app.pages.dev/tickets/ticket/ga_pass#";
export const EXISTING_FACTORY = `1726853571421-factory.testnet`;
export const ADMIN_ACCOUNTS = [SIGNER_ACCOUNT];

export const PREMADE_TICKET_DATA = [
  {
    name: "Jake",
    email: "",
  },
  {
    name: "Kiana",
    email: "foo",
  },
  {
    name: "Min",
    email: "foo",
  },
  {
    name: "Benji",
    email: "foo",
  },
  {
    name: "David",
    email: "foo",
  },
];
