import {gql} from "urql";

export const ApiVersionQuery = gql `
    query {
        apiVersion
    }
`