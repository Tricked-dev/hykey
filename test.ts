const endpoints = [
  "/player",
  "/recentgames",
  "/status",
  "/skyblock/profile",
  "/skyblock/profiles",
];

const test_uuids = [
  "41a9b6aa-168a-4be8-8df8-cac17daf7384",
  "7678ee20-99ef-466f-9f49-7c3109f175ba",
  "f7c77d99-9f15-4a66-a87d-c4a51ef30d19",
  "b876ec32-e396-476b-a115-8438d83c67d4",
  "28667672-0390-4498-9b00-19b14a2c34d6",
  "063f9bdf-047b-47ef-85b6-533ff1dfd69b",
  "daf2a9b9-2326-4cc2-b1a9-d49194886c70",
  "e14b92fd-e4ad-4cfd-a08a-8360ca78d4ac",
  "34a35801-3895-4aae-a7e5-fbb52e575e3d",
  "afc2469d-bd63-40d6-8382-f87dd24fbec8",
  "c558970d-ca73-43a7-9083-825bacfa65c7",
  "14ea1a9c-3511-4b16-8e1c-e98ca5ca3f72",
  "a335c6e5-eac2-4e07-9e00-4b10e4783bd8",
];

Promise.all(
  endpoints.map((endpoint) => {
    return Promise.all(
      test_uuids.map((uuid) => {
        return fetch(`http://127.0.0.1:4000${endpoint}?uuid=${uuid}`).then(
          (r) => r.text(),
        );
      }),
    );
  }),
);
