import { gql } from '@apollo/client';
import * as Apollo from '@apollo/client';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
const defaultOptions = {} as const;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export type BoardGameResult = {
  __typename?: 'BoardGameResult';
  id: Scalars['Int'];
  maxPlayers?: Maybe<Scalars['Int']>;
  minPlayers?: Maybe<Scalars['Int']>;
  name: Scalars['String'];
  playtime?: Maybe<Scalars['Int']>;
  year?: Maybe<Scalars['Int']>;
};

export type Query = {
  __typename?: 'Query';
  /** Returns the sum of a and b */
  add: Scalars['Int'];
  games: Array<BoardGameResult>;
};


export type QueryAddArgs = {
  a: Scalars['Int'];
  b: Scalars['Int'];
};


export type QueryGamesArgs = {
  limit?: InputMaybe<Scalars['Int']>;
};

export type GamesQueryVariables = Exact<{ [key: string]: never; }>;


export type GamesQuery = { __typename?: 'Query', games: Array<{ __typename?: 'BoardGameResult', id: number, name: string }> };

export type Games2QueryVariables = Exact<{ [key: string]: never; }>;


export type Games2Query = { __typename?: 'Query', games: Array<{ __typename?: 'BoardGameResult', id: number, playtime?: number | null }> };


export const GamesDocument = gql`
    query Games {
  games {
    id
    name
  }
}
    `;

/**
 * __useGamesQuery__
 *
 * To run a query within a React component, call `useGamesQuery` and pass it any options that fit your needs.
 * When your component renders, `useGamesQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGamesQuery({
 *   variables: {
 *   },
 * });
 */
export function useGamesQuery(baseOptions?: Apollo.QueryHookOptions<GamesQuery, GamesQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<GamesQuery, GamesQueryVariables>(GamesDocument, options);
      }
export function useGamesLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<GamesQuery, GamesQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<GamesQuery, GamesQueryVariables>(GamesDocument, options);
        }
export type GamesQueryHookResult = ReturnType<typeof useGamesQuery>;
export type GamesLazyQueryHookResult = ReturnType<typeof useGamesLazyQuery>;
export type GamesQueryResult = Apollo.QueryResult<GamesQuery, GamesQueryVariables>;
export const Games2Document = gql`
    query Games2 {
  games {
    id
    playtime
  }
}
    `;

/**
 * __useGames2Query__
 *
 * To run a query within a React component, call `useGames2Query` and pass it any options that fit your needs.
 * When your component renders, `useGames2Query` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGames2Query({
 *   variables: {
 *   },
 * });
 */
export function useGames2Query(baseOptions?: Apollo.QueryHookOptions<Games2Query, Games2QueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<Games2Query, Games2QueryVariables>(Games2Document, options);
      }
export function useGames2LazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<Games2Query, Games2QueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<Games2Query, Games2QueryVariables>(Games2Document, options);
        }
export type Games2QueryHookResult = ReturnType<typeof useGames2Query>;
export type Games2LazyQueryHookResult = ReturnType<typeof useGames2LazyQuery>;
export type Games2QueryResult = Apollo.QueryResult<Games2Query, Games2QueryVariables>;

      export interface PossibleTypesResultData {
        possibleTypes: {
          [key: string]: string[]
        }
      }
      const result: PossibleTypesResultData = {
  "possibleTypes": {}
};
      export default result;
    