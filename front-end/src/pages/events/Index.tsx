import Events from "@/components/Events";

function EventsPage() {
  return (
    <div>
      <h1 className="text-lg font-bold p-4"> Upcoming Events</h1>
      <Events showAll={true} />
    </div>
  );
}

export default EventsPage;
