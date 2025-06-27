import React from "react";

const About: React.FC = () => {
  return (
    <div style={{ padding: "2rem", maxWidth: "800px", margin: "0 auto" }}>
      <h1 className="text-2xl font-bold">About Fort Wayne Open Hack</h1>
      <p>
        Fort Wayne Open Hack is a weekly meetup for developers, designers,
        tinkerers, and tech enthusiasts of all skill levels. Whether you’re
        building a side project, learning to code, or just want to hang out and
        talk shop, you’re welcome here. We meet every Wednesday evening at
        Electric Works — bring your laptop, your curiosity, and whatever you’re
        working on. There’s no formal agenda, just an open space to connect,
        collaborate, and share ideas. No sign-up, no pressure — just show up.
        Come code, build, break, and learn with us.
      </p>
    </div>
  );
};

export default About;
