import { useEffect, useState } from 'react';
import { formatDistance } from 'date-fns';
import { motion } from 'framer-motion';
import { IoSearch } from 'react-icons/io5';
import { twMerge as merge } from 'tailwind-merge';

import { SearchResultStatus } from '../util';

type UndergraduateCourse = {
  id: number;
  code: string;
  number: number;
  suffix: string | null;

  title: string;
  description: string;
  credits: number;
  min_credits: number | null;

  GA: boolean; // Arts
  GHW: boolean; // Health and Wellness
  GH: boolean; // Humanities
  GN: boolean; // Natural Sciences
  GQ: boolean; // Quantification
  GS: boolean; // Social and Behavioral Sciences
  GWS: boolean; // Writing and Speaking

  ITD: boolean; // Inter-Domain
  LKD: boolean; // Linked

  FYS: boolean; // First-Year Seminar
  IC: boolean; // International Cultures
  US: boolean; // United States Cultures
  WCC: boolean; // Writing Across the Curriculum

  // B.A. Requirements
  BA: boolean; // Bachelor of Arts: Arts
  BH: boolean; // Bachelor of Arts: Humanities
  BN: boolean; // Bachelor of Arts: Natural Sciences
  BO: boolean; // Bachelor of Arts: Other Cultures
  BQ: boolean; // Bachelor of Arts: Quantification
  BS: boolean; // Bachelor of Arts: Social and Behavioral Sciences
  BF1: boolean; // Bachelor of Arts: Foreign/World Lang (12th Unit)
  BF2: boolean; // Bachelor of Arts: 2nd Foreign/World Language (All)
};

export default function Landing({
  stats,
}: {
  stats?: {
    age: number;
    record_count: number;
  };
}) {
  const [minimize, setMinimize] = useState(false);
  const [search, setSearch] = useState('');
  const [searchResults, setSearchResults] = useState<{
    data: UndergraduateCourse[];
    status: SearchResultStatus;
  }>({ data: [], status: SearchResultStatus.Empty });

  useEffect(() => {
    handleSearch(search);
  }, [search]);

  function handleSearch(query: string) {
    if (!query) {
      return;
    }

    // test data for now
    // fake waiting
    setSearchResults({ data: [], status: SearchResultStatus.Loading });

    fetch(
      'http://localhost:3001/api/search/courses?' +
        new URLSearchParams({
          query,
        }),
    )
      .then((res) => res.json())
      .then((data) =>
        setSearchResults({
          data,
          status: SearchResultStatus.Success,
        }),
      )
      .catch((err) => {
        console.error(err);
        setSearchResults({ data: [], status: SearchResultStatus.Empty });
      });
  }

  useEffect(() => {
    if (!search) {
      setMinimize(false);
      setSearchResults({ data: [], status: SearchResultStatus.Empty });
    }
  }, [search]);

  return (
    <div className="flex w-full">
      <motion.div
        className={merge(
          minimize ? 'pt-4' : 'justify-center',
          'mx-auto flex size-full max-w-xl flex-col items-center',
        )}
      >
        <motion.div
          layout="preserve-aspect"
          className={merge(
            !minimize && '-mt-48',
            'flex w-full flex-col items-center gap-4',
          )}
        >
          <motion.h1
            initial={{ opacity: 1 }}
            animate={search ? 'hidden' : 'visible'}
            transition={{ duration: 0.1 }}
            variants={{
              hidden: {
                opacity: 0,
                transitionEnd: {
                  display: 'none',
                },
              },
              visible: {
                display: 'block',
                opacity: 1,
              },
            }}
            onAnimationComplete={() => {
              setMinimize(!!search);
            }}
            className={merge(
              'relative select-none text-4xl font-bold text-psu-300',
            )}
          >
            Better Bulletin
            <span className="absolute ml-2 whitespace-nowrap text-sm font-normal">
              v2.1 alpha
            </span>
          </motion.h1>
          <div className="flex h-auto w-full items-center rounded-full bg-gray-100 px-4 py-3">
            <IoSearch className="mr-2 size-5 text-gray-400" />
            <input
              type="text"
              placeholder="Search for courses..."
              //   placeholder="Search for majors, courses, etc..."
              className="w-full bg-transparent text-psu-100 focus:border-blue-500 focus:outline-none"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>
          <motion.p
            initial={{ opacity: 1 }}
            animate={search ? 'hidden' : 'visible'}
            transition={{ duration: 0.1 }}
            variants={{
              hidden: {
                opacity: 0,
                transitionEnd: {
                  display: 'none',
                },
              },
              visible: {
                display: 'block',
                opacity: 1,
              },
            }}
            onAnimationComplete={() => {
              setMinimize(!!search);
            }}
            className={merge(
              'relative select-none text-center text-sm text-psu-300',
            )}
          >
            {stats &&
              `Cache last updated ${formatDistance(stats.age, Date.now(), { addSuffix: true })}.`}{' '}
            <br />
            {stats && `Retrieved ${stats.record_count} courses.`}
          </motion.p>
          <motion.div
            initial="hidden"
            animate={minimize ? 'visible' : 'hidden'}
            variants={{
              visible: {
                opacity: 1,
                transition: {
                  duration: 0.1,
                  delay: 0.25,
                },
              },
              hidden: {
                opacity: 0,
                transition: {
                  duration: 0.1,
                  delay: 0.25,
                },
              },
            }}
            className={merge(
              minimize ? 'block' : 'hidden',
              'w-full overflow-auto',
            )}
          >
            {(() => {
              switch (searchResults.status) {
                case SearchResultStatus.Empty:
                  return null;
                case SearchResultStatus.Loading:
                  return <p className="text-center">loading...</p>;
                case SearchResultStatus.Success:
                  return searchResults.data.length > 0 ? (
                    <ul className="flex flex-col gap-4 pb-12">
                      {searchResults.data.map((result, i) => (
                        <a
                          href={`/course/${result.id}`}
                          key={i}
                          className="flex flex-col gap-1 rounded-md bg-gray-100 p-2  hover:bg-blue-100"
                        >
                          <h2 className="text-xl font-bold text-psu-100 ">
                            {`${result.code} ${result.number}${result.suffix ? result.suffix : ''}`}
                          </h2>
                          <h3 className="italic text-psu-100">
                            {result.title}
                          </h3>
                          <p className="line-clamp-2 overflow-hidden text-ellipsis text-psu-200">
                            {result.description}
                          </p>
                          <div className="mt-4 flex w-full">
                            <div className="flex flex-1 gap-2">
                              {(() => {
                                const attributes = [];
                                result.GA &&
                                  attributes.push({
                                    title: 'GA',
                                    color: '#facfcf',
                                  });
                                result.GHW && attributes.push('GHW');
                                result.GH && attributes.push('GH');
                                result.GN && attributes.push('GN');
                                result.GQ && attributes.push('GQ');
                                result.GS && attributes.push('GS');
                                result.GWS && attributes.push('GWS');

                                result.ITD && attributes.push('Inter-Domain');
                                result.LKD && attributes.push('Linked');

                                result.FYS &&
                                  attributes.push('First Year Seminar');
                                result.IC &&
                                  attributes.push('International Cultures');
                                result.US &&
                                  attributes.push('United States Cultures');
                                result.WCC &&
                                  attributes.push(
                                    'Writing Across the Curriculum',
                                  );

                                return attributes.map((attr, i) => (
                                  <div
                                    style={
                                      typeof attr === 'object'
                                        ? {
                                            backgroundColor: attr.color,
                                          }
                                        : {}
                                    }
                                    className="inline-block rounded-xl border border-psu-100 px-2 py-1 text-sm text-psu-100"
                                    key={i}
                                  >
                                    {typeof attr === 'string'
                                      ? attr
                                      : attr.title}
                                  </div>
                                ));
                              })()}
                            </div>
                            <div className="flex justify-end">
                              <div className="rounded-xl px-2 py-1 text-sm text-psu-100">
                                {result.min_credits
                                  ? `${result.min_credits}-${result.credits} CREDITS`
                                  : `${result.credits} CREDITS`}
                              </div>
                            </div>
                          </div>
                        </a>
                      ))}
                    </ul>
                  ) : (
                    <p className="text-center">No search results found!</p>
                  );
                default:
                  return null;
              }
            })()}
          </motion.div>
        </motion.div>
      </motion.div>
    </div>
  );
}
