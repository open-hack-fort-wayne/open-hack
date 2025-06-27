import React from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "./ui/card";
import { Button } from "./ui/button";

function Events() {
  const dummyEvents = [
    {
      title: "Event 1",
      description: "Description 1",
      date: "2021-01-01",
      time: "10:00",
      location: "Location 1",
    },
    {
      title: "Event 2",
      description: "Description 2",
      date: "2021-01-02",
      time: "10:00",
      location: "Location 2",
    },
  ];
  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-row gap-4">
        {dummyEvents.map((event) => (
          <EventCard key={event.title} {...event} />
        ))}
      </div>
      <Button className="w-fit">View All Events</Button>
    </div>
  );
}

type eventCardProps = {
  title: string;
  description: string;
  date: string;
  time: string;
  location: string;
};

function EventCard({
  title,
  description,
  date,
  time,
  location,
}: eventCardProps) {
  return (
    <Card className="max-w-64 min-w-52">
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-2">
        <p>{date}</p>
        <p>{time}</p>
        <p>{location}</p>
        <Button>Sign Up</Button>
      </CardContent>
    </Card>
  );
}

export default Events;
