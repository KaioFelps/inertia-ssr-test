import ReactDOMServer from 'react-dom/server'
import { createInertiaApp } from '@inertiajs/react'
import React from "react"
import server from "@inertiajs/core/server";

function render(page: any) {
  return createInertiaApp({
    page,
    render: ReactDOMServer.renderToString,
    resolve: (name) => {
      const pages = import.meta.glob('./pages/**/*.tsx', { eager: true })
      const page: any = pages[`./pages/${name}.tsx`]

      return page
    },
    setup: ({ App, props }) => {
      return <App {...props} />
    },
  })
}

server(async (page) => await render(page), 1000)