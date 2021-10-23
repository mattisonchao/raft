/*
 *     Licensed to the Apache Software Foundation (ASF) under one
 *     or more contributor license agreements.  See the NOTICE file
 *     distributed with this work for additional information
 *     regarding copyright ownership.  The ASF licenses this file
 *     to you under the Apache License, Version 2.0 (the
 *     "License"); you may not use this file except in compliance
 *     with the License.  You may obtain a copy of the License at
 *
 *       http://www.apache.org/licenses/LICENSE-2.0
 *
 *     Unless required by applicable law or agreed to in writing,
 *     software distributed under the License is distributed on an
 *     "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 *     KIND, either express or implied.  See the License for the
 *     specific language governing permissions and limitations
 *     under the License.
 */
use std::time::{Instant, Duration};
use std::any::Any;
use crate::server::Server;
use std::thread;
use std::sync::Mutex;
use std::borrow::Borrow;
use std::ops::Add;
use self::rand::Rng;

extern crate rand;

const DEBUG: u64 = 1;

enum CMState {
    Follower = 0,
    Candidate = 1,
    Leader = 2,
    Dead = 3,
}

struct LogEntry {
    term: u64,
    command: dyn Any,
}

pub struct ConsensusModule {
    // id is the server ID of this CM.
    id: u64,
    // peerIds lists the IDs of our peers in the cluster.
    peer_ids: Vec<u64>,

    // server is the server containing this CM. It's used to issue RPC calls
    // to peers.
    server: Server,

    // Persistent Raft state on all servers
    current_term: Mutex<u64>,
    voted_for: u64,
    log: Vec<LogEntry>,

    // Volatile Raft state on all servers
    state: CMState,
    election_reset_event: Mutex<Instant>,
}

impl ConsensusModule {
    fn new_consensus_module(id: u64, peer_ids: Vec<u64>, server: Box<Server>, ready: &dyn Fn() -> ()) -> ConsensusModule {
        let cm = ConsensusModule {
            id,
            peer_ids,
            server: *server,
            current_term: Mutex::new(0),
            voted_for: -1,
            log: vec![],
            state: CMState::Follower,
            election_reset_event: Mutex::new(Instant::now()),
        };
        thread::spawn(|| {
            ready();
            let mut election_reset_event = cm.election_reset_event.lock().unwrap();
            *election_reset_event = Instant::now();
        });

        return cm;
    }
    // electionTimeout generates a pseudo-random election timeout duration.
    fn election_timeout(&self) -> Instant {
        return Instant::add(Instant::now(), Duration::from_millis(150 + rand::thread_rng().gen_range(0, 150)));
    }
    // runElectionTimer implements an election timer. It should be launched whenever
    // we want to start a timer towards becoming a candidate in a new election.
    //
    // This function is blocking and should be launched in a separate goroutine;
    // it's designed to work for a single (one-shot) election timer, as it exits
    // whenever the CM state changes from follower/candidate or the term changes.
    fn run_election_timer(&self) {
        let timeout_duration = self.election_timeout();
        let current_term = *self.current_term.lock().unwrap();
    }
}