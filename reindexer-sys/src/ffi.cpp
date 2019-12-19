#include <string>
#include <iostream>
#include <fstream>
#include <vector>
#include <deque>
#include "core/reindexer.h"
#include "core/item.h"
#include "client/reindexer.h"
#include "client/queryresults.h"
#include "tools/errors.h"

//#include "core/keyvalue/key_string.h"
//#include "core/keyvalue/variant.h"

#include <chrono>
using std::chrono::seconds;

using namespace reindexer;
using namespace std;

class Iterator {
public:
    Iterator(reindexer::client::QueryResults::Iterator start, reindexer::client::QueryResults::Iterator end) {
        //this->start = start;
        this->end = end;
        this->current = start;
        this->iter = false;
    }
    //reindexer::client::QueryResults::Iterator start;
    reindexer::client::QueryResults::Iterator current;
    reindexer::client::QueryResults::Iterator end;
    bool iter;;
};

extern "C" {

void reindexer_test() {
    using std::chrono::milliseconds;
    const string default_namespace = "test_namespace";

    auto db = std::shared_ptr<Reindexer> (new Reindexer);

    Error err = db->OpenNamespace(default_namespace, StorageOpts().Enabled(false));
    //ASSERT_TRUE(err.ok()) << err.what();
    cout << err.ok() << " " << err.what() << endl;

    err = db->AddIndex(default_namespace, {"id", "hash", "int", IndexOpts().PK()});
    //ASSERT_TRUE(err.ok()) << err.what();

    err = db->AddIndex(default_namespace, {"value", "text", "string", IndexOpts()});
    //ASSERT_TRUE(err.ok()) << err.what();

    Item item(db->NewItem(default_namespace));
    //ASSERT_TRUE(item.Status().ok()) << item.Status().what();

    err = item.FromJSON(R"_({"id":1234, "value" : "value"})_");
    //ASSERT_TRUE(err.ok()) << err.what();

    err = db->WithTimeout(milliseconds(1000)).Insert(default_namespace, item);
    //ASSERT_TRUE(err.ok()) << err.what();

    err = db->WithTimeout(milliseconds(100)).Commit(default_namespace);
    //ASSERT_TRUE(err.ok()) << err.what();

    QueryResults qr;
    err = db->WithTimeout(milliseconds(1000)).Select(Query(default_namespace), qr);
    //ASSERT_TRUE(err.ok()) << err.what();
    //ASSERT_EQ(qr.Count(), 1);

    // check item consist and check case insensitive access to field by name
    Item selItem = qr.begin().GetItem();
    //ASSERT_NO_THROW(ASSERT_EQ(selItem["id"].As<int>(), 1234));
    //ASSERT_NO_THROW(ASSERT_EQ(selItem["value"].As<string>(), "value"));
    cout << "id: " << selItem["id"].As<int>() << endl;
    string_view a = selItem.GetJSON();
    cout << "item json: " << a << endl;

    qr.Clear();
    err = db->WithTimeout(milliseconds(1000)).Delete(Query(default_namespace), qr);
    //ASSERT_TRUE(err.ok()) << err.what();
}

void reindexer_client_test() {
    cout << "reindexer_client_test" << endl;
    const string default_namespace = "test_namespace";
    reindexer::client::ReindexerConfig config;
    config.ConnectTimeout = seconds(3);
    config.RequestTimeout = seconds(3);
    auto db = new reindexer::client::Reindexer(config);
    const char *const kDefaultRPCServerAddr = "127.0.0.1:6534";
    auto res = db->Connect(string("cproto://") + kDefaultRPCServerAddr + "/test_db");
    cout << "res: " << res.ok() << endl;
    Error err = db->OpenNamespace(default_namespace, StorageOpts().Enabled(false));
    //ASSERT_TRUE(err.ok()) << err.what();
    cout << err.ok() << " " << err.what() << endl;

    err = db->AddIndex(default_namespace, {"id", "hash", "int", IndexOpts().PK()});
    //ASSERT_TRUE(err.ok()) << err.what();
    cout << err.ok() << " " << err.what() << endl;

    //err = db->AddIndex(default_namespace, {"value", "text", "string", IndexOpts()});
    //ASSERT_TRUE(err.ok()) << err.what();

    //Item item(db->NewItem(default_namespace));
    auto item = db->NewItem(default_namespace);
    //ASSERT_TRUE(item.Status().ok()) << item.Status().what();

    err = item.FromJSON(R"_({"id":1234, "value" : "value"})_");
    //ASSERT_TRUE(err.ok()) << err.what();

    err = db->WithTimeout(milliseconds(1000)).Upsert(default_namespace, item);
    //ASSERT_TRUE(err.ok()) << err.what();

    err = db->WithTimeout(milliseconds(100)).Commit(default_namespace);
    //ASSERT_TRUE(err.ok()) << err.what();

    //auto query = "a";
    Query query1 = Query(default_namespace).Where("id", CondEq, "1234");
    string_view query2 = "SELECT * FROM test_namespace";

    reindexer::client::QueryResults qr;
    //err = db->WithTimeout(milliseconds(1000)).Select(query2, qr);
    err = db->Select(query2, qr);
    //ASSERT_TRUE(err.ok()) << err.what();
    //ASSERT_EQ(qr.Count(), 1);
    cout << err.ok() << " " << err.what() << endl;
    //cout << "qr.Count()2: " << qr.Count() << endl;

    // check item consist and check case insensitive access to field by name
    //reindexer::client::Item selItem = qr.begin().GetItem();
    //ASSERT_NO_THROW(ASSERT_EQ(selItem["id"].As<int>(), 1234));
    //ASSERT_NO_THROW(ASSERT_EQ(selItem["value"].As<string>(), "value"));
    //cout << "id: " << selItem["id"].As<int>() << endl;
    //string_view a = selItem.GetJSON();
    //cout << "item json: " << a << endl;
    //auto selItem = qr.GetItem();

    for (auto it : qr) {
        //reindexer::client::Item ritem(it.GetItem());
        //auto ritem = it.GetItem();
        //auto json = ritem.GetJSON();
        WrSerializer ser;
        //auto ok = it.GetJSON(ser);
        auto ok = it.GetJSON(ser, false);
        //auto ok = it.GetCJSON(ser);
        string json(ser.Slice());
        //string json(ser.c_str());
        //EXPECT_TRUE(json == R"xxx({"id":"key2","locale":"ru","nested":{"name":"name2","count":2}})xxx");
        cout << "item: " << ok.ok() << json << endl;
    }

//    for (auto it : res) {
//        reindexer::client::Item ritem(it.GetItem());
//        std::string outBuf = "";
//        string_view a = ritem.GetJSON();
//        //for (auto idx = 1; idx < ritem.NumFields(); idx++) {
//        //    outBuf += "\t";
//        //    outBuf += ritem[idx].As<string>();
//        //}
//        cout << outBuf << std::endl;
//    }

    //qr.Clear();
    //err = db->WithTimeout(milliseconds(1000)).Delete(Query(default_namespace), qr);
}

reindexer::client::Reindexer *reindexer_client_new() {
    //cout << "reindexer_client_new" << endl;
    //const string default_namespace = "test_namespace";
    reindexer::client::ReindexerConfig config;
    config.ConnectTimeout = seconds(5*60);
    config.RequestTimeout = seconds(5*60);
    return new reindexer::client::Reindexer(config);
}

void reindexer_client_destroy(reindexer::client::Reindexer *db) {
    if (db != nullptr) {
        delete db;
        db = nullptr;
    }
}

// cproto://127.0.0.1:6534/test_db
bool reindexer_client_connect(reindexer::client::Reindexer *db, const char* dsn) {
    //cout << "reindexer_client_connect: " << dsn << endl;
    auto err = db->Connect(string(dsn));
    return err.ok();
}

bool reindexer_client_open_namespace(reindexer::client::Reindexer *db, const char* ns) {
    //cout << "reindexer_client_open_namespace: " << ns << endl;
    Error err = db->OpenNamespace(string(ns), StorageOpts().Enabled(false));
    return err.ok();
}

bool reindexer_client_insert(reindexer::client::Reindexer *db, const char* ns, const char* data) {
    reindexer::client::Item item(db->NewItem(ns));
    //ASSERT_TRUE(item.Status().ok()) << item.Status().what();

    //Error err = item.FromJSON(R"_({"id":1234, "value" : "value"})_");
    Error err = item.FromJSON(data);
    //ASSERT_TRUE(err.ok()) << err.what();
    if (!err.ok()) {
        return false;
    }

    //err = db->WithTimeout(milliseconds(1000)).Insert(ns, item);
    //err = db->WithTimeout(milliseconds(1000*60*10)).Insert(ns, item);
    err = db->Insert(ns, item);
    //ASSERT_TRUE(err.ok()) << err.what();
    if (!err.ok()) {
        return false;
    }

    //err = db->WithTimeout(milliseconds(1000*60*10)).Commit(ns);
    err = db->Commit(ns);
    return err.ok();
}

bool reindexer_client_update(reindexer::client::Reindexer *db, const char* ns, const char* data) {
    reindexer::client::Item item(db->NewItem(ns));
    //ASSERT_TRUE(item.Status().ok()) << item.Status().what();

    //Error err = item.FromJSON(R"_({"id":1234, "value" : "value"})_");
    Error err = item.FromJSON(data);
    //ASSERT_TRUE(err.ok()) << err.what();
    if (!err.ok()) {
        return false;
    }

    //err = db->WithTimeout(milliseconds(1000)).Insert(ns, item);
    //err = db->WithTimeout(milliseconds(1000*60*10)).Update(ns, item);
    err = db->Update(ns, item);
    //ASSERT_TRUE(err.ok()) << err.what();
    if (!err.ok()) {
        return false;
    }

    //err = db->WithTimeout(milliseconds(1000*60*10)).Commit(ns);
    err = db->Commit(ns);
    return err.ok();
}

bool reindexer_client_upsert(reindexer::client::Reindexer *db, const char* ns, const char* data) {
    reindexer::client::Item item(db->NewItem(ns));
    //ASSERT_TRUE(item.Status().ok()) << item.Status().what();

    //Error err = item.FromJSON(R"_({"id":1234, "value" : "value"})_");
    Error err = item.FromJSON(data);
    //ASSERT_TRUE(err.ok()) << err.what();
    if (!err.ok()) {
        return false;
    }

    //err = db->WithTimeout(milliseconds(1000)).Insert(ns, item);
    //err = db->WithTimeout(milliseconds(1000*60*10)).Upsert(ns, item);
    err = db->Upsert(ns, item);
    //ASSERT_TRUE(err.ok()) << err.what();
    if (!err.ok()) {
        return false;
    }

    //err = db->WithTimeout(milliseconds(1000*60*10)).Commit(ns);
    err = db->Commit(ns);
    return err.ok();
}

bool reindexer_client_delete(reindexer::client::Reindexer *db, const char* ns, const char* data) {
    reindexer::client::Item item(db->NewItem(ns));
    //ASSERT_TRUE(item.Status().ok()) << item.Status().what();

    //Error err = item.FromJSON(R"_({"id":1234, "value" : "value"})_");
    Error err = item.FromJSON(data);
    //ASSERT_TRUE(err.ok()) << err.what();
    if (!err.ok()) {
        return false;
    }

    //err = db->WithTimeout(milliseconds(1000)).Insert(ns, item);
    //err = db->WithTimeout(milliseconds(1000*60*10)).Delete(ns, item);
    err = db->Delete(ns, item);
    //ASSERT_TRUE(err.ok()) << err.what();
    if (!err.ok()) {
        return false;
    }

    //err = db->WithTimeout(milliseconds(1000*60*10)).Commit(ns);
    err = db->Commit(ns);
    return err.ok();
}

bool reindexer_client_select(reindexer::client::Reindexer *db, reindexer::client::QueryResults *qr, const char* query) {
    Error err = db->Select(query, *qr);
    //cout << err.ok() << " " << err.what() << endl;
    return err.ok();
}

reindexer::client::QueryResults *query_results_new() {
    return new reindexer::client::QueryResults();
}

void query_results_destroy(reindexer::client::QueryResults *qr) {
    if (qr != nullptr) {
        delete qr;
        qr = nullptr;
    }
}

int query_results_count(reindexer::client::QueryResults *qr) {
    return qr->Count();
}

Iterator* query_results_iter(reindexer::client::QueryResults *qr) {
    return new Iterator(qr->begin(), qr->end());
}

bool query_results_iterator_next(Iterator *it) {
    if (it->iter) {
        if (it->current == it->end) {
            return false;
        }
        ++it->current;
    } else {
        it->iter = true;
    }
    if (it->current == it->end) {
        return false;
    }
    return it->current.Status().ok();
}

bool query_results_iter_get_json(Iterator *it, char *output) {
    WrSerializer ser;
    auto ok = it->current.GetJSON(ser, false);
    //string json(ser.Slice());
    strcpy(output, ser.c_str());
    return ok.ok();
}

void query_results_iter_destroy(Iterator *it) {
    if (it != nullptr) {
        delete it;
        it = nullptr;
    }
}

//WrSerializer *wr_serializer_new() {
//    return new WrSerializer();
//}
//

//void wr_serializer_destroy(WrSerializer *ser) {
//    delete ser;
//    ser = nullptr;
//}

}