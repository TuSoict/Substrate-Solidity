import { EmptyLayout, MainLayout } from '../components/layout'
import { Web3Provider } from '../components/provider'
import '../styles/globals.css'

function MyApp({ Component, pageProps }) {
  const Layout = MainLayout?? EmptyLayout
  return(
    <Web3Provider>
      <Layout>
        <Component {...pageProps} />
      </Layout>
    </Web3Provider>
   
  )

}

export default MyApp
