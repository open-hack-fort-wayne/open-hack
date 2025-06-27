import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "./ui/card";
import { Button } from "./ui/button";
import { Link } from "react-router-dom";

function Events({ showAll = false }: { showAll?: boolean }) {
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
    {
      title: "Event 3",
      description: "Description 3",
      date: "2021-01-03",
      time: "10:00",
      location: "Location 3",
    },
    {
      title: "Event 4",
      description: "Description 4",
      date: "2021-01-04",
      time: "10:00",
      location: "Location 4",
    },
  ];
  return (
    <div className="flex flex-col gap-4 p-4">
      <div className="flex flex-row flex-wrap gap-4">
        {dummyEvents.map((event, idx) => {
          if (showAll !== true && idx > 2) return null;
          return <EventCard key={event.title} {...event} />;
        })}
      </div>
      {!showAll && (
        <Link to="/events">
          <Button className="w-fit">View All Events</Button>
        </Link>
      )}
      {showAll && (
        <>
          <h2 className="text-lg font-bold">Past Events</h2>
          <div className="flex flex-row flex-wrap gap-4">
            {dummyEvents.map((event) => {
              return <EventCard key={event.title} {...event} />;
            })}
          </div>
        </>
      )}
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
