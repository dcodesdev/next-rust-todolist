import { AxiosError } from "axios"
import { toast } from "sonner"

export const toastError = (e: unknown | string) => {
  if (e instanceof AxiosError) {
    toast.error(e.response?.data?.message || e.message)
  } else if (e instanceof Error) {
    toast.error(e.message)
  } else if (typeof e === "string") {
    toast.error(e)
  } else {
    toast.error("An error occurred")
  }
}
