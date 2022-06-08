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

export type GamesQueryVariables = Exact<{
  limit?: InputMaybe<Scalars['Int']>;
}>;


export type GamesQuery = { __typename?: 'Query', games: Array<{ __typename?: 'BoardGameResult', id: number, name: string }> };

export type GamesPlaytimeQueryVariables = Exact<{ [key: string]: never; }>;


export type GamesPlaytimeQuery = { __typename?: 'Query', games: Array<{ __typename?: 'BoardGameResult', id: number, playtime?: number | null }> };


export const GamesDocument = gql`
    query Games($limit: Int) {
  games(limit: $limit) {
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
 *      limit: // value for 'limit'
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
export const GamesPlaytimeDocument = gql`
    query GamesPlaytime {
  games {
    id
    playtime
  }
}
    `;

/**
 * __useGamesPlaytimeQuery__
 *
 * To run a query within a React component, call `useGamesPlaytimeQuery` and pass it any options that fit your needs.
 * When your component renders, `useGamesPlaytimeQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGamesPlaytimeQuery({
 *   variables: {
 *   },
 * });
 */
export function useGamesPlaytimeQuery(baseOptions?: Apollo.QueryHookOptions<GamesPlaytimeQuery, GamesPlaytimeQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<GamesPlaytimeQuery, GamesPlaytimeQueryVariables>(GamesPlaytimeDocument, options);
      }
export function useGamesPlaytimeLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<GamesPlaytimeQuery, GamesPlaytimeQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<GamesPlaytimeQuery, GamesPlaytimeQueryVariables>(GamesPlaytimeDocument, options);
        }
export type GamesPlaytimeQueryHookResult = ReturnType<typeof useGamesPlaytimeQuery>;
export type GamesPlaytimeLazyQueryHookResult = ReturnType<typeof useGamesPlaytimeLazyQuery>;
export type GamesPlaytimeQueryResult = Apollo.QueryResult<GamesPlaytimeQuery, GamesPlaytimeQueryVariables>;

      export interface PossibleTypesResultData {
        possibleTypes: {
          [key: string]: string[]
        }
      }
      const result: PossibleTypesResultData = {
  "possibleTypes": {}
};
      export default result;
    