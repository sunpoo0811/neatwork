export interface CasdoorConfig {
    serverUrl: string;
    clientId: string;
    organizationName: string;
    appName: string;
    redirectPath: string;
}

export const config: CasdoorConfig = {
    serverUrl: "http://localhost:7001/",
    clientId: "c5cbfaf6792eb12163a4",
    organizationName: "built-in",
    appName: "neat_work_space",
    redirectPath: "/callback",
};
// export const config: CasdoorConfig = {
//     serverUrl: "https://your-casdoor-server.com",
//     clientId: "your-client-id",
//     organizationName: "your-org-name",
//     appName: "your-app-name",
//     redirectPath: "/callback",
// };