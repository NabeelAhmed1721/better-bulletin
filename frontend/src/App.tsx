import { useEffect, useState } from 'react';
import { AnimatePresence } from 'framer-motion';
import { Redirect, Route, Switch } from 'wouter';

import CoursePage from './components/CoursePage';
import Landing from './components/Landing';

type Stats = {
  age: number;
  record_count: number;
};

export default function App() {
  const [stats, setStats] = useState<Stats | undefined>();

  useEffect(() => {
    fetch('http://localhost:3001/api/stats')
      .then((res) => res.json())
      .then(setStats)
      .catch(console.error);
  }, []);

  return (
    // full screen
    <main className="flex h-screen w-screen">
      <AnimatePresence>
        <Switch>
          <Route path="/course/:id">
            {(params) => <CoursePage id={parseInt(params.id)} />}
          </Route>
          <Route path="/">{stats && <Landing stats={stats} />}</Route>
          <Route path="*">{() => <Redirect to="/" />}</Route>
        </Switch>
      </AnimatePresence>
    </main>
  );
}
