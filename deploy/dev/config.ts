import { CreationConfig, PremadeTicketData } from "../types";

export const GLOBAL_NETWORK = "testnet";

export const SIGNER_ACCOUNT = "minqi.testnet";
export const CLEANUP_CONTRACT = false;
export const CREATION_CONFIG: CreationConfig = {
  deployContract: true,

  // TICKETS
  addTickets: true,
  premadeTickets: true,

  // ACCOUNTS
  createSponsors: true,
  createWorker: true,
  createAdmin: true,

  nftDrops: false,
  tokenDrops: false,
  scavDrops: false,
  multichainDrops: true,
};

export const SITE_BASE_URL = "https://development.keypom-events-app.pages.dev";
export const EXISTING_FACTORY = `1728587692346-factory.testnet`;
export const ADMIN_ACCOUNTS = [SIGNER_ACCOUNT];

export const PREMADE_TICKET_DATA: PremadeTicketData = [
  {
    name: "Test User 1",
    email: "",
  },
  {
    name: "Test User 2",
    email: "foo",
  },
  {
    name: "Test User 3",
    email: "foo",
  },
  {
    name: "Test User 4",
    email: "foo",
  },
  {
    name: "Test User 5",
    email: "foo",
  },
  {
    name: "Test User 6",
    email: "foo",
  },
  {
    name: "Test User 7",
    email: "foo",
  },
];
