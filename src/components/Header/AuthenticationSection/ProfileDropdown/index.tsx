import React from 'react';

export default function ProfileDropdown({ image }: { image: string }) {
  const [open, setOpen] = React.useState(false);

  const handleOpen = () => {
    setOpen(!open);
  };

  return (
    <button type="button" onClick={handleOpen} className="relative">
      {/* eslint-disable-next-line @next/next/no-img-element */}
      <img src={image} alt="Authenticated User" className="h-10 rounded" />
      {open && (
        <ul className="absolute right-0 mt-2 space-y-2 rounded border border-ctp-mauve bg-ctp-surface0 px-4 py-2">
          <li>
            <a href="/dashboard">Dashboard</a>
          </li>
          <li>
            <a href="/api/auth/logout">Logout</a>
          </li>
        </ul>
      )}
    </button>
  );
}
