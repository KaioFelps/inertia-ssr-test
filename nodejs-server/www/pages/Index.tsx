import React from "react"
import { Head } from "@inertiajs/react"

type Props = {
    auth: {
        user: string
    }
}

export default function Index({auth: {user}}: Props) {
    return (
        <>
            <Head>
                <title>Hello inertia!</title>
                <meta name="description" content="Just a mocked head... Ha!" />
            </Head>
            <h1>Hello 4000 {user}</h1>
        </>
    )
}
