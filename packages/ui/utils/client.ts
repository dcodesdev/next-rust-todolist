import axios, { AxiosRequestConfig } from "axios"
import Cookies from "js-cookie"
import { logout } from "./user"
import { toastError } from "./toast"

export const client = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL || "http://localhost:5000",
  headers: {
    Authorization: `Bearer ${Cookies.get("token")}`,
  },
})

client.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      logout()
    }

    return Promise.reject(error)
  }
)

export const get = <T = any>(url: string, config?: AxiosRequestConfig) =>
  client
    .get<T>(url, config)
    .then((r) => r.data)
    .catch(toastError)

export const post = <T = any>(
  url: string,
  body?: any,
  config?: AxiosRequestConfig<T>
) =>
  client
    .post<T>(url, body, config)
    .then((r) => r.data)
    .catch((e) => {
      toastError(e)
      throw Error(e)
    })
