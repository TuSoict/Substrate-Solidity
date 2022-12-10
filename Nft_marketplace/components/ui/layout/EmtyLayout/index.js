import Link from "next/link";

const EmtyLayout = ({ children }) => {
  return (
    <div>
      {children}
      <Link href="/">
        <button>Go to Home Page</button>
      </Link>
    </div>
  );
};

export default EmtyLayout;
