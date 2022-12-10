import { Footer, Header } from "../../common";

const MainLayout = ({ children }) => {
  return (
    <div>
      <Header />
      <div className="fit max-w-[1440px] mx-auto">{children}</div>
      <Footer />
    </div>
  );
};

export default MainLayout;
