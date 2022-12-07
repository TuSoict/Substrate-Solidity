import { Header,Footer } from "../common";

export default function MainLayout({children}){
    return(
        <>
            <Header />
            {children}
            <div className="fit"></div>
            <Footer />
        </>
    )
}