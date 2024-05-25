import Cookies from "js-cookie"

export const logout = (redirect = true) => {
  Cookies.remove("token")

  if (redirect) {
    window.location.href = "/login"
  }
}
