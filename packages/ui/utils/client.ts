import axios from "axios"
import Cookies from "js-cookie"
import { logout } from "./user"

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
