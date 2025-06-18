#include <algorithm>
#include <bitset>
#include <iostream>
#include <queue>
#include <vector>
#include<tuple>
using namespace std;


vector<pair<int,int>> adj[10001];
bitset<10001> visited;
tuple<int,int,int> pred[10001];


void bfs() {
    queue<int> q;
    pred[0] = {-1,-1,-1};
    q.push(0);
    while(!q.empty()) {
        int l = q.front();
        q.pop();
        for (auto [m, r] : adj[l]) {
            if (visited[r])continue;
            visited[r] = true;
            q.push(r);
            pred[r] = {l,m,r};
        }
    }
}


int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    int n;
    cin>>n;
    for (int i = 0;i<n;i++) {
        int l,r,m;
        cin>>l>>m>>r;
        adj[l] .push_back( make_pair(m,r));
    }
    bfs();
    if (pred[0] == tuple(-1,-1,-1) ) {
        cout<<"BRAK"<<endl;
    }
    else {
        vector<tuple<int,int,int>> path = {};
        int curr = 0;
        do {
            tuple<int,int,int> p = pred[curr];
            auto [l,m,r] = p;
            path.push_back(p);
            curr = l;
        } while (curr!=0);
        reverse(path.begin(),path.end());
        cout<<path.size()<<endl;
        for (auto [l,m,r] : path) {
            cout<<l<<" "<<m<<" "<<r<<endl;
        }
    }
return 0;
}