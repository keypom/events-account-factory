export const GLOBAL_NETWORK = "testnet";

export const SIGNER_ACCOUNT = "benjiman.testnet";
export const CREATION_CONFIG = {
  deployContract: false,

  // TICKETS
  addTickets: false,
  premadeTickets: false,

  // ACCOUNTS
  createSponsors: false,
  createWorker: false,

  premadeDrops: true,
};
export const NUM_TICKETS_TO_ADD = 5;

export const TICKET_URL_BASE = "http://localhost:5173/tickets/ticket/ga_pass#";
export const EXISTING_FACTORY = `1725907929524-factory.testnet`;
export const ADMIN_ACCOUNTS = [SIGNER_ACCOUNT];

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

export const PREMADE_DROP_DATA = [
  {
    drop_data: {
      name: "illia talk poap",
    },
    nft_metadata: {
      title: "Illia Talk POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "opening ceremony poap",
    },
    nft_metadata: {
      title: "Opening Ceremony POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "closing ceremony poap",
    },
    nft_metadata: {
      title: "Closing Ceremony POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "Keypom ceremony poap",
    },
    nft_metadata: {
      title: "Keypom Booth POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "nuffle waffles ceremony poap",
    },
    nft_metadata: {
      title: "Nuffle Waffles POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "proximity ceremony poap",
    },
    nft_metadata: {
      title: "Proximity Booth POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "snowden ceremony poap",
    },
    nft_metadata: {
      title: "Snowden Talk POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "winer ceremony poap",
    },
    nft_metadata: {
      title: "Eric Winer Talk POAP",
      description:
        "Here are some instructions on how to retrieve this collectible. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas placerat mauris turpis, vel consequat mi ultricies eu. Quisque ligula neque, placerat ut dui.",
      media:
        "https://builders.mypinata.cloud/ipfs/QmYXJ89PFMYEcPbqA8DTbmzZu7qKrgKBUjS6kHUovHA3k7",
    },
  },
  {
    drop_data: {
      name: "10 Tokens",
    },
    token_amount: "10",
  },
  {
    drop_data: {
      name: "20 Tokens",
    },
    token_amount: "20",
  },
  {
    drop_data: {
      name: "25 Tokens",
    },
    token_amount: "25",
  },
];
