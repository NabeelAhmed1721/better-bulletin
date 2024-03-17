import { Link } from 'wouter';

export default function Header() {
  return (
    <header className="pointer-events-none fixed z-50 flex h-16 w-full items-center p-2 backdrop-blur-sm">
      <div className="flex-1" />
      <div className="flex-1 text-center">
        <Link
          className="pointer-events-auto text-xl font-bold text-blue-500 hover:text-blue-700"
          href="/"
        >
          Return Home
        </Link>
      </div>
      <div className="flex-1" />
    </header>
  );
}
