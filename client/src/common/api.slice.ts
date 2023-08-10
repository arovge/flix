import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react'
import type { GetMediaResponse } from './types'

export const flixApi = createApi({
  reducerPath: 'flixApi',
  baseQuery: fetchBaseQuery({ baseUrl: 'http://localhost:8000' }),
  endpoints: (builder) => ({
    getMedia: builder.query<GetMediaResponse, void>({
      query: () => 'media',
      providesTags: [{ type: 'Media', id: 'LIST' }]
    }),
  }),
  tagTypes: ['Media']
})


export const { useGetMediaQuery } = flixApi
