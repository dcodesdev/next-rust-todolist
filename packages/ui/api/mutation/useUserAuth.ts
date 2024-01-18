import { post } from "@/utils";
import { useMutation } from "@tanstack/react-query";
import { LoginUserResponse, RegisterUserResponse } from "server";
import Cookies from "js-cookie";
import { z } from "zod";

const userLoginSchema = z.object({
  email: z
    .string({
      required_error: "Email is required",
    })
    .email(),
  password: z
    .string({
      required_error: "Password is required",
    })
    .min(1, "Password is required"),
  name: z.string().optional(),
});

export const useUserAuth = (isRegister = false) => {
  return useMutation({
    mutationFn: async (body: any) => {
      const parsed = userLoginSchema.safeParse(body);

      if (!parsed.success) {
        throw new Error(parsed.error.issues[0].message);
      }

      const { email, password } = parsed.data;

      return isRegister
        ? post<RegisterUserResponse>("/user", {
            email,
            password,
          }).then((r) => {
            Cookies.set("token", r.token);
            window.location.href = "/dashboard";
          })
        : post<LoginUserResponse>("/user/login", { email, password }).then(
            (r) => {
              Cookies.set("token", r.token);
              window.location.href = "/dashboard";
            },
          );
    },
  });
};
