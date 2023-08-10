import { useGetMediaQuery } from "./common/api.slice"

export default function MediaList() {
    const getMediaQuery = useGetMediaQuery()

    if (getMediaQuery.isLoading || getMediaQuery.isFetching) {
        return <span>Loading...</span>
    }

    if (getMediaQuery.isError || getMediaQuery.data == null) {
        return <span>Error</span>
    }

    return (
        <>
            {getMediaQuery.data.media.map(m => (
                <div key={m.id}>
                    <span>{m.name}</span>
                </div>
            ))}
        </>
    )
}
