import React from "react";
import developers from "../assets/Developers.png";
import Events from "@/components/Events";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
const Home: React.FC = () => {
  return (
    <div className="flex flex-col gap-2 p-12">
      <h1 className="mt-4 text-2xl font-bold">
        Welcome to Fort Wayne Open Hack
      </h1>

      <div className="flex md:flex-row flex-col gap-4 relative h-96 md:h-[550px] justify-center md:items-center md:justify-start">
        <Card className="w-1/2 md:w-1/3 z-10 m-4">
          <CardHeader>
            <CardTitle>About Us</CardTitle>
          </CardHeader>
          <CardContent>
            <p>
              We meet every Wednesday evening at Electric Works — bring your
              laptop, your curiosity, and whatever you're working on. There's no
              formal agenda, just an open space to connect, collaborate, and
              share ideas.
            </p>
          </CardContent>
        </Card>
        <div className="absolute w-full h-96 md:h-full overflow-hidden rounded-lg">
          <img
            src={developers}
            alt="Developers"
            className="w-full h-full object-cover object-center"
          />
        </div>
      </div>
      <h2 className="text-lg font-bold mt-12">Upcoming Events</h2>
      <Events />
    </div>
  );
};

export default Home;
