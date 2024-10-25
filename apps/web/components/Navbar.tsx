"use client";
import { logout } from "@/utils/user";
import { Container } from "./Container";
import { Button } from "./ui/button";
import Link from "next/link";
import { Separator } from "./ui/separator";

export const Navbar = () => {
  return (
    <Container className="p-3">
      <div className="flex justify-between items-center gap-2">
        <Link href="/dashboard">
          <Button>Dashboard</Button>
        </Link>

        <Button onClick={() => logout()} variant="ghost">
          Logout
        </Button>
      </div>
      <Separator />
    </Container>
  );
};
