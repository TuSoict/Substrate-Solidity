import Link from "next/link";
import { MainLayout } from "../../components/ui/layout";

export default function BlogPage() {
  return (
    <>
      <div>Blog page</div>
      <Link href="/">
        <button>Go to Home Page</button>
      </Link>
    </>
  );
}

BlogPage.Layout = MainLayout;
