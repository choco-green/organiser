import { UserProvider } from '@auth0/nextjs-auth0/client';
import type { Metadata } from 'next';
import { Roboto } from 'next/font/google';
import React from 'react';
import './globals.css';

// Components
import Header from '@/components/Header';

export const metadata: Metadata = {
  title: 'Create Next App',
  description: 'Generated by create next app',
};

const roboto = Roboto({
  weight: ['100', '300', '400', '500', '700', '900'],
  subsets: ['latin'],
});

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={roboto.className}>
      <UserProvider>
        <body className="theme-frappe flex flex-col items-center bg-ctp-base text-ctp-text">
          <Header />
          {children}
        </body>
      </UserProvider>
    </html>
  );
}
