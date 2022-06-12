import type {NextPage} from "next";
import {useGamesQuery} from "../../graphql/games.gql";
import {useRouter} from "next/router";

const Games: NextPage = () => {
    const router = useRouter()
    const {username} = router.query
    const {loading, error, data} = useGamesQuery({variables: {username}});

    return (
        <main>
            <h1 className="text-3xl font-bold underline">Games for {username}</h1>
            <div>
                {loading && "Loading..."}
                {error && <p>Error: {error.message}</p>}
                <>
                    {data?.games?.map((game) => (
                        <p key={game.id}>{game.name}</p>
                    ))}
                </>
            </div>
        </main>
    );
};

export default Games;
