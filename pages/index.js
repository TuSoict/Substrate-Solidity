import Head from 'next/head'
import Image from 'next/image'
import MainLayout from '../components/layout/EmptyLayout'
import { useWeb3 } from '../components/provider'
import styles from '../styles/Home.module.css'

export default function Home() {
  const {contract} = useWeb3();
  console.log(contract);
  return (
    <div>
      <h1 className='text-xl text-red-400'>Hello World</h1> 
    </div>
  )
}

Home.Layout = MainLayout