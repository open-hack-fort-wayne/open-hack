import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import React from "react";

function Login() {
  return (
    <div className="flex flex-col gap-2 items-center justify-center h-[80%]">
      <h2>Login</h2>
      <form>
        <div className="flex flex-col gap-2">
          <Input type="text" placeholder="Email" />
          <Input type="password" placeholder="Password" />
          <Button type="submit">Login</Button>
        </div>
        <button type="button">Sign Up</button>
      </form>
    </div>
  );
}

export default Login;
