import Link from "next/link";
import { MainLayout } from "../components/ui/layout";
import { useAccount } from "../components/hooks/web3";

export default function Home() {

  return (
    <>
      <div className="text-3xl">Home Page</div>
      <Link href="/blog">
        <button>Go to Blog Page</button>
      </Link>
    </>
  );
}

Home.Layout = MainLayout;
