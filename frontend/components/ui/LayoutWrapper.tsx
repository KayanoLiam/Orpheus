"use client";

import { usePathname } from 'next/navigation';
import Navbar from './Navbar';

export default function LayoutWrapper({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();
  
  // 在登录页面、条款页面和隐私政策页面不显示Navbar
  const shouldShowNavbar = pathname !== '/login' && pathname !== '/terms' && pathname !== '/privacy';
  
  return (
    <>
      {shouldShowNavbar && <Navbar />}
      {children}
    </>
  );
}